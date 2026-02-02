import Foundation

var greeting = "Hello, playground"

print(greeting)

func incr(_ x: Int) -> Int {
    return x + 1
}

incr(2)

func square(_ x: Int) -> Int {
    return x * x
}

square(2)


extension Int {
    func incr() -> Int {
        return self + 1
    }

    func square() -> Int {
        return self * self
    }
}

2.incr()
2.incr().square()


precedencegroup ForwardApplication {
    associativity: left
}

infix operator |>: ForwardApplication

func |> <A, B>(a: A, f: (A) -> B) -> B {
    return f(a)
}

2 |> incr
2 |> incr |> square

infix operator >>>
func >>> <A,B,C>(f: @escaping (A) -> B, g: @escaping (B) -> C) -> ((A) -> C) {
    return { a in
        g(f(a))
    }
}

incr >>> square
square >>> incr

2 |> incr
