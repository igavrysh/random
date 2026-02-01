/*
See the LICENSE.txt file for this sample’s licensing information.

Abstract:
A sticker view data model for managing imported and processed photos,
  as well as their current state.
*/

import SwiftUI
import PhotosUI

@MainActor
@Observable
class StickerViewModel {
    /// An array of items for the picker's selected photos.
    var selection = [SelectedPhoto]()

    /// A dictionary that maps the photo ID to its processed version.
    var processedPhotos = [SelectedPhoto.ID: ProcessedPhoto]()

    /// An array of photos that didn't process successfully.
    var invalidPhotos: [SelectedPhoto.ID] = []

    let stickerService: StickerService
    let photoProcessor: PhotoProcessor

    init() {
        let stickerService = StickerService()
        self.stickerService = stickerService
        self.photoProcessor = PhotoProcessor(stickerService: stickerService)
        if !cachedSelection.isEmpty {
            self.selection = cachedSelection.map {
                SelectedPhoto(itemIdentifier: $0)
            }
        }
    }

    func loadPhoto(_ item: SelectedPhoto) async {
        var data: Data? = try? await item.loadTransferable(type: Data.self)

        if let cachedData = getCachedData(for: item.id) { data = cachedData }

        guard let data else { return }
        processedPhotos[item.id] = await photoProcessor.process(data: data)

        cacheData(item.id, data)
    }

    func processAllPhotos() async {
        await withTaskGroup { group in
            for item in selection {
                guard processedPhotos[item.id] == nil else { continue }
                group.addTask {
                    let data = await self.getData(for: item)
                    let photo = await PhotoProcessor(stickerService: self.stickerService).process(data: data)
                    return photo.map { ProcessedPhotoResult(id: item.id, processedPhoto: $0)}
                }
            }

            for await result in group {
                if let result {
                    processedPhotos[result.id] = result.processedPhoto
                }
            }

        }
    }
}
