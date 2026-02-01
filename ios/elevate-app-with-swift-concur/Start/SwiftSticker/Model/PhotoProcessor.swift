/*
See the LICENSE.txt file for this sample’s licensing information.

Abstract:
A data model that extracts a mask and dominant colors from a source image to create a destination image and color scheme.
*/

import SwiftUI
import PhotosUI
import Vision
import CoreImage.CIFilterBuiltins

nonisolated struct PhotoProcessor {

    let colorExtractor = ColorExtractor()

    var stickerService: StickerService

    @concurrent
    func process(data: Data) async -> ProcessedPhoto? {
        async let sticker = extractSticker(from: data)
        let colors = extractColors(from: data)

        guard let sticker = await sticker, let colors = colors else { return nil }

        return ProcessedPhoto(sticker: sticker, colorScheme: colors)
    }

    private func extractColors(from data: Data) -> PhotoColorScheme? {
        return colorExtractor.extractColors(from: data)
    }

    private func extractSticker(from data: Data) async -> Image? {
        guard let userSelectedImage = UIImage(data: data) else { return nil }

#if targetEnvironment(simulator)
        do {
            let extracted = try await stickerService.extractSticker(from: userSelectedImage)
            return extracted.map(Image.init)
        } catch {
            print("Extraction Error: \(error)")
            return nil
        }
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
