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

    @State private var activeCardId: Int = 1

    var body: some View {
        ScrollView {
            //LazyVGrid(columns: columns, spacing: 0) {
            VStack(spacing: 0) {
                ForEach(ids, id: \.self) { id in
                    RotatingCard(id: id, activeCardId: $activeCardId)
                        .frame(height: 250)
                        .zIndex(activeCardId == id ? 1.0 : 0.0)
                }
            }
        }
    }
}

struct RotatingCard: View {
    let id: Int
    @Binding var activeCardId: Int

    @State var showFront: Bool = true
    @State private var pct: Double = 0

    var body: some View {
        ZStack {
            if showFront {
                Front(id: id)
            } else {
                Back(id: id)
                    .rotation3DEffect(.degrees(180), axis: (x: 0, y: 1, z: 0)) // Un-flip the text
            }
        }

        .modifier(RotateEffect(pct: pct, showFront: $showFront))

        .onTapGesture {
            activeCardId = id
            print("active card: \(activeCardId)")
            withAnimation(.spring(response: 0.6, dampingFraction: 0.8)) {
                // Toggles back and forth
                pct = (pct == 0) ? 1 : 0
            }
        }
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
                .inset(by: 2)

                .stroke(Color.blue, lineWidth: 4) // Apply stroke to the shape
        )
    }
}

struct Back: View {
    var id: Int
    var onTap: (() -> Void)? = nil

    var body: some View {
        ZStack {
            Color.red
            VStack(spacing: 8) {
                Text("Back for \(id)")
                Image("amethyst")
                    .resizable()
                    .scaledToFit()
                    .frame(width: 30, height: 30)
            }
        }
        .cornerRadius(20)       // Clips the background to rounded corners
        .overlay(
            RoundedRectangle(cornerRadius: 20) // Use a shape matching the corners
                .inset(by: 2)
                .stroke(Color.green, lineWidth: 4) // Apply stroke to the shape
        )
    }
}

struct RotateEffect: GeometryEffect {
    var pct: Double

    var animatableData: Double {
        get { pct }
        set { pct = newValue }
    }

    @Binding var showFront: Bool // Pass the state back up

    func effectValue(size: CGSize) -> ProjectionTransform {
        var angle: Double

        if pct < 0.25 {
            angle = 0
        } else if pct >= 0.25 && pct <= 0.75 {
            angle = 180 * ((pct-0.25) / 0.5)
        } else { // if pct > 0.75 {
            angle = 180
        }

        print("angle: \(angle)")
        //DispatchQueue.main.async {
        //    self.front = self.angle >= 90 && self.angle < 180
       // }

        let isFront = angle < 90

        DispatchQueue.main.async {
            if self.showFront != isFront {
                self.showFront = isFront
            }
        }

        let radians = CGFloat(Angle(degrees: angle).radians)
        var transform3d = CATransform3DIdentity
        // -1.0 / 500.0 is a standard "natural" perspective.
        transform3d.m34 = -1.0 / 1000.0
        let translation = CATransform3DMakeTranslation(size.width/2, size.height/2, 0)
        let rotation = CATransform3DRotate(transform3d, radians, 0, 1, 0) // Y-axis rotation
        let reversal = CATransform3DMakeTranslation(-size.width/2, -size.height/2, 0)

        let targetScale = 0.9

        var zoom = CATransform3DIdentity
        if pct <= 0.25 {
            // 3. Scale (Shrink to 20%)
            let scale: CGFloat = 1.0-(pct/0.25) * (1-targetScale)
            zoom = CATransform3DScale(zoom, scale, scale, 1.0)
        } else if pct <= 0.75 {
            zoom = CATransform3DScale(zoom, targetScale, targetScale, 1.0)
        } else if pct > 0.75 {
            let scale: CGFloat = targetScale + ((pct-0.75)/0.25) * (1-targetScale)
            zoom = CATransform3DScale(zoom, scale, scale, 1.0)
        }

        let finalTransform = CATransform3DConcat(reversal, CATransform3DConcat(zoom, CATransform3DConcat(rotation, translation)))

        return ProjectionTransform(finalTransform)
    }
}

#Preview {
    ContentView()
}
