//
//  FilmRoll.swift
//  TestRotatingView
//
//  Created by new on 2/19/26.
//

import SwiftUI

struct FilmRoll: View {
    @State private var selectedItemIndex: Int = 5 // <-- here

    var body: some View {
        ZStack {
            Color.black.ignoresSafeArea()
            ScrollView {
                LazyVGrid(columns: [GridItem()]) {
                    ForEach(1..<13) { index in
                        Rectangle()
                            .fill(Color(
                                red: (selectedItemIndex == index) ? 1 : Double(index)/24.0 + 0.5,
                                green: 0,
                                blue: 0))
                            .zIndex(selectedItemIndex == index ? 1 : 0)
                            .frame(width: 180, height: 180)
                            .border(.black)
                            .rotationEffect(.degrees(45))
                            .onTapGesture {
                                selectedItemIndex = index
                                print("----> selected item \(selectedItemIndex)")
                            }
                    }
                    .id(selectedItemIndex) // <-- here
                }
            }
        }
    }
}

#Preview {
    FilmRoll()
}

