/*
See the LICENSE.txt file for this sample’s licensing information.

Abstract:
A data model that extracts a mask and dominant colors from a source image to create a destination image and color scheme.
*/

import SwiftUI
import PhotosUI
import Vision
import CoreImage.CIFilterBuiltins

struct PhotoProcessor {

    let colorExtractor = ColorExtractor()

    let stickerService = StickerService()

    func process(data: Data) -> ProcessedPhoto? {
        let sticker = extractSticker(from: data)
        let colors = extractColors(from: data)

        guard let sticker = sticker, let colors = colors else { return nil }

        return ProcessedPhoto(sticker: sticker, colorScheme: colors)
    }

    private func extractColors(from data: Data) -> PhotoColorScheme? {
        return colorExtractor.extractColors(from: data)
    }

    private func extractSticker(from data: Data) -> Image? {
        guard let userSelectedImage = UIImage(data: data) else { return nil }

#if targetEnvironment(simulator)

        // Use a class that is definitely Sendable and not isolated
        final class ResultBox: @unchecked Sendable {
            private(set) var image: UIImage?
            private(set) var finished = false

            // Moving the mutation to a function bypasses the isolation check
            func store(_ result: UIImage?) {
                self.image = result
                self.finished = true
            }
        }

        let box = ResultBox()
        let service = self.stickerService // Capture local reference

        Task.detached(priority: .userInitiated) {
            do {
                print("calling service.extractSticker")
                // Call the async service
                let extracted = try await service.extractSticker(from: userSelectedImage)

                // This is the line that was throwing the error.
                // Because ResultBox is @unchecked Sendable, this is now legal.
                await box.store(extracted)
            } catch {
                print("Extraction Error: \(error)")
                await box.store(nil)
            }
        }

        Thread.sleep(forTimeInterval: 1)


        // BLOCKING LOOP: Force the thread to wait but keep RunLoop spinning
        // This allows the M1 Simulator to handle internal Vision/CoreML events
        while !box.finished {
            RunLoop.current.run(mode: .default, before: Date(timeIntervalSinceNow: 1))
            print("still running")
        }

        return box.image.map(Image.init)
#else
        guard let image = CIImage(data: data) else { return nil }
        let handler = VNImageRequestHandler(ciImage: image)
        let request = VNGenerateForegroundInstanceMaskRequest()

        do {
            try handler.perform([request])

            guard let result = request.results?.first else { return nil }

            let maskPixelBuffer = try result.generateScaledMaskForImage(
                forInstances: result.allInstances,
                from: handler
            )
            let mask = CIImage(cvPixelBuffer: maskPixelBuffer)
            let extent = mask.extent

            let minDimension = min(extent.width, extent.height)
            let scaledRadius = max(1, Int(minDimension * 0.02))

            let dilatedMask = mask
                .applyingFilter("CIMorphologyMaximum", parameters: [
                    "inputRadius": scaledRadius
                ])

            let whiteBackground = CIImage(color: .white)
                .cropped(to: extent)
                .applyingFilter("CIBlendWithMask", parameters: [
                    "inputMaskImage": dilatedMask
                ])

            let subject = image
                .applyingFilter("CIBlendWithMask", parameters: [
                    "inputMaskImage": mask
                ])

            let sticker = subject.composited(over: whiteBackground)

            guard let cgImage = CIContext()
                .createCGImage(sticker, from: sticker.extent) else {
                return nil
            }
            return Image(decorative: cgImage, scale: 1.0)
        } catch {
            print("Unable to extract foreground: \(error)")
            return nil
        }
#endif

    }

    func applyMockMask(to inputImage: CIImage) -> CIImage {
        // 1. Create a circular mask in the center
        let center = CGPoint(x: inputImage.extent.midX, y: inputImage.extent.midY)
        let radius = min(inputImage.extent.width, inputImage.extent.height) * 0.4

        // 2. Use a radial gradient to create a white circle on a black background
        let mask = CIFilter(name: "CIRadialGradient", parameters: [
            "inputCenter": CIVector(cgPoint: center),
            "inputRadius0": radius,
            "inputRadius1": radius + 1,
            "inputColor0": CIColor.white,
            "inputColor1": CIColor.clear
        ])?.outputImage?.cropped(to: inputImage.extent)

        // 3. Blend the original image with the mask
        let blendFilter = CIFilter(name: "CIBlendWithMask", parameters: [
            "inputImage": inputImage,
            "inputMaskImage": mask!
        ])

        return blendFilter?.outputImage ?? inputImage
    }
}
