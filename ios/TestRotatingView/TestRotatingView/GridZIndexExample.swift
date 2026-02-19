//
//  GridZIndexExample.swift
//  TestRotatingView
//
//  Created by new on 2/19/26.
//

import SwiftUI

struct GridZIndexExample: View {
    @State private var selectedCardID: UUID?

    let columns = [GridItem(.flexible()), GridItem(.flexible()), GridItem(.flexible())]
    let cards = (0...8).map { i in
        Card(id: UUID(), color: Color(hue: Double(i) / 9.0, saturation: 0.8, brightness: 0.8))
    }


    var body: some View {
//        LazyVGrid(columns: columns, spacing: -10) {
        VStack {
            ForEach(cards) { card in
                CardView(card: card, selectedCardID: $selectedCardID)
                    .padding(-10)
                    .zIndex(zIndexValue(card: card))

            }
        }
        .padding(-10)
    }

    func zIndexValue(card: Card) -> Double {
        selectedCardID == card.id ? 1.0 : 0.0 // Bring selected card to front
    }
}

struct Card: Identifiable {
    var id: UUID
    var color: Color
}

struct CardView: View {
    let card: Card
    @Binding var selectedCardID: UUID?

    // Determine the zIndex dynamically
    var zIndexValue: Double {
        selectedCardID == card.id ? 1.0 : 0.0 // Bring selected card to front
    }

    var body: some View {
        RoundedRectangle(cornerRadius: 10)
            .fill(card.color)
            .frame(height: 100)
            .shadow(radius: 5)
            .onTapGesture {
                withAnimation(.easeInOut) {
                    selectedCardID = card.id
                }
            }
            // Apply the zIndex modifier
            .zIndex(zIndexValue)
            .overlay(
                Text(selectedCardID == card.id ? "Top" : "Normal")
                    .foregroundColor(.white)
            )
    }
}

#Preview {
    GridZIndexExample()
}
