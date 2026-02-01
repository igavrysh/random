//
//  StickerService.swift
//  SwiftSticker
//
//  Created by new on 1/31/26.
//  Copyright © 2026 Apple. All rights reserved.
//

import CoreML
import Vision
import UIKit

nonisolated class StickerService {
    private var visionModel: VNCoreMLModel?
    private let context = CIContext() // Re-use context for better performance

    @MainActor
    init() {
        let config = MLModelConfiguration()

        #if targetEnvironment(simulator)
        config.computeUnits = .cpuOnly
        #else
        config.computeUnits = .all
        #endif
        do {
            // Ensure 'u2netp' matches your imported .mlmodel file name
            print("[model] loading u2netp model")
            let model = try u2netp(configuration: config)
            print("[model] u2netp model loaded")
            visionModel = try VNCoreMLModel(for: model.model)
            print("[model] vision model loaded")
        } catch {
            print("[model] model failed to load: \(error)")
        }
    }

    /// Generates a sticker by removing the background
    func extractSticker(from inputImage: UIImage) async throws -> UIImage? {
        print("[model]extractSticker start")

        guard let visionModel = visionModel,
              let ciImage = CIImage(image: inputImage)
        else {
            print("[model]throwing error")
            throw NSError(domain: "StickerService", code: 1, userInfo: [NSLocalizedDescriptionKey: "Model not loaded"])
        }

        print("[model]sending request")

        return try await withCheckedThrowingContinuation { continuation in
            let request = VNCoreMLRequest(model: visionModel) { request, error in
                print("[model]starting request")
                if let error = error {
                    print("[model]error:\(error)")
                    continuation.resume(throwing: error)
                    return
                }

                print("[model]applying mask")

                guard let results = request.results as? [VNPixelBufferObservation],
                      let maskPixelBuffer = results.first?.pixelBuffer else {
                    continuation.resume(returning: nil)
                    return
                }

                print("[model]applying mask")

                let sticker = self.applyMask(maskBuffer: maskPixelBuffer, to: ciImage)
                continuation.resume(returning: sticker)
            }

            request.imageCropAndScaleOption = .scaleFill

            let handler = VNImageRequestHandler(ciImage: ciImage, options: [:])

            // Perform on main queue so the main RunLoop can service Vision/CoreML callbacks
            // (required for simulator; background GCD threads have no RunLoop)
            do {
                try handler.perform([request])
            } catch {
                continuation.resume(throwing: error)
            }
        }
    }

    private func applyMask(maskBuffer: CVPixelBuffer, to originalImage: CIImage) -> UIImage? {
        var maskImage = CIImage(cvPixelBuffer: maskBuffer)

        // Scale mask to match original image size
        let scaleX = originalImage.extent.width / maskImage.extent.width
        let scaleY = originalImage.extent.height / maskImage.extent.height
        maskImage = maskImage.transformed(by: CGAffineTransform(scaleX: scaleX, y: scaleY))

        // Blend: Keep original pixels only where mask is white
        let filter = CIFilter(name: "CIBlendWithMask")
        filter?.setValue(originalImage, forKey: kCIInputImageKey)
        filter?.setValue(maskImage, forKey: kCIInputMaskImageKey)

        if let output = filter?.outputImage,
           let cgImage = context.createCGImage(output, from: output.extent) {
            return UIImage(cgImage: cgImage)
        }
        return nil
    }
}
