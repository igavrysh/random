//
//  ContentView.swift
//  TestRotatingView
//
//  Created by new on 2/16/26.
//

import SwiftUI

struct ContentView: View {
    let ids = [1,2,3,4]
    let columns = Array(
        repeating: GridItem(.flexible(minimum: 250, maximum: 450), spacing: 16),
        count: 1)


    var body: some View {
        ScrollView {
            LazyVGrid(columns: columns, spacing: 0) {

                ForEach(ids, id: \.self) { id in
                    RotatingCard(id: id)
                        .frame(height: 250)
                }
            }
        }
    }
}

struct RotatingCard: View {
    let id: Int
    @State var showFront: Bool = true
    @State var rotateFromFront: Bool = false

    var body: some View {
        let binding = Binding<Bool>(
            get: { self.showFront },
            set: { self.showFront = $0 }
        )

        ZStack {
            if showFront {
                Front(id: id, onTap: {
                    withAnimation(.easeInOut(duration: 1.0)) {
                        self.rotateFromFront = true
                    }
                })
            } else {
                Back(id: id, onTap: {
                    withAnimation(.easeInOut(duration: 1.0)) {
                        self.rotateFromFront = false
                    }
                })
            }
        }
//        .modifier(
//            RotateEffect(
//                flipped: binding,
//                angle: 180,// rotateFromFront ? 180 : 0,
//                axis: (x: 1, y: 5)
//            )
//        )
    }
}

struct Front: View {
    let id: Int
    var onTap: (() -> Void)? = nil

    var body: some View {
        ZStack {
            Color.gray

            VStack(spacing: 8) {
                Image("amethyst")
                    .resizable()
                    .scaledToFit()
                    .frame(width: 150)
                Text("Amethyst \(id)".uppercased())
                    .fontWeight(.semibold)
                    .fontDesign(.rounded)
                    .font(.title)
                Button {
                    onTap?()
                } label: {
                    Text("Tap to flip")
                        .font(.body)
                        .foregroundColor(.secondary)
                }
            }
        }
        .cornerRadius(20)       // Clips the background to rounded corners
        .overlay(
            RoundedRectangle(cornerRadius: 20) // Use a shape matching the corners
                .stroke(Color.blue, lineWidth: 4) // Apply stroke to the shape
        )
    }
}

struct Back: View {
    var id: Int
    var onTap: (() -> Void)? = nil

    var body: some View {
        ZStack {
            Color.white
            VStack(spacing: 8) {
                Text("Back for \(id)")
                Image("amethyst")
                    .resizable()
                    .scaledToFit()
                    .frame(width: 30, height: 30)
            }
        }
    }
}

struct RotateEffect: GeometryEffect {
    var animatableData: Double {
        get { angle }
        set { angle = newValue }
    }

    @Binding var flipped: Bool
    var angle: Double
    let axis: (x: CGFloat, y: CGFloat)

    func effectValue(size: CGSize) -> ProjectionTransform {
        print("angle: \(angle) flipped: \(flipped)")
        DispatchQueue.main.async {
            self.flipped = self.angle >= 90 && self.angle < 180
        }

        let a = CGFloat(Angle(degrees: angle).radians)

        var transform3d = CATransform3DIdentity
        //transform3d.m34 = -1 / max(size.width, size.height)

        //transform3d = CATransform3DRotate(transform3d, a, axis.x, axis.y, 0)
        transform3d = CATransform3DRotate(transform3d, a, 0, axis.y, 0)
        transform3d = CATransform3DTranslate(transform3d, -size.width/2.0, -size.height/2.0, 0)

        let affineTransform = ProjectionTransform(CGAffineTransform(translationX: size.width/2.0, y: size.height/2.0))

        return ProjectionTransform(transform3d).concatenating(affineTransform)
    }
}

#Preview {
    ContentView()
}
