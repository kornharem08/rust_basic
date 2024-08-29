## SetUp Manual
    1. สร้าง ไฟล์ hello.rs
    2. rustc hello.rs
    3. ./hello

## Create with Cargo
    1. cargo new hello-rust
    2. cd /hello-rust
    3. cargo run


## Scalar Types (ชนิดข้อมูลพื้นฐาน)

### Integer (จำนวนเต็ม)
ชนิดข้อมูลที่ใช้เก็บตัวเลขจำนวนเต็ม เช่น `i32`, `u8`, `isize`

```rust
let x: i32 = 42;   // จำนวนเต็ม 32 บิตที่มีเครื่องหมาย
let y: u8 = 255;   // จำนวนเต็ม 8 บิตที่ไม่มีเครื่องหมาย
```

### Floating-Point (จำนวนจริง)
ชนิดข้อมูลที่ใช้เก็บตัวเลขจำนวนจริง เช่น f32, f64
```rust
let x: f64 = 3.14; // จำนวนจริง 64 บิต
```

### Boolean (บูลีน)
ใช้เก็บค่า true หรือ false
```rust
let is_active: bool = true;
```

### Character (อักขระ)
ใช้เก็บตัวอักษรหนึ่งตัว โดยใช้ char
```rust
let letter: char = 'A';
```

## Compound Types (ชนิดข้อมูลผสม)

### Tuple (ทูเพิล)
เป็นกลุ่มของค่าที่มีหลายประเภท สามารถเก็บค่าต่างชนิดในกลุ่มเดียวกันได้
```rust
let tuple: (i32, f64, char) = (42, 3.14, 'A');
let (x, y, z) = tuple; // Destructuring
```

### Array (อาร์เรย์)
เป็นกลุ่มของค่าที่มีประเภทเดียวกัน โดยมีขนาดคงที่
```rust
let arr: [i32; 3] = [1, 2, 3]; // อาร์เรย์ของจำนวนเต็มขนาด 3 ตัว
let first_element = arr[0];    // เข้าถึงค่าในอาร์เรย์
```

### String Types (ชนิดข้อมูลสตริง)
&str (String slice)
ตัวอ้างอิงถึงชุดของตัวอักษรที่ไม่สามารถเปลี่ยนแปลงได้

```rust
let greeting: &str = "Hello, world!";
```

### String
ตัวอักษรที่สามารถเปลี่ยนแปลงขนาดได้

```rust
let mut s = String::from("Hello");
s.push_str(", world!"); // เพิ่มข้อความ
```

## Reference Types (ชนิดข้อมูลตัวอ้างอิง)

### &T (Reference)
ตัวอ้างอิงไปยังค่าโดยไม่สามารถเปลี่ยนแปลงค่าได้
```rust
let x = 42;
let y: &i32 = &x; // ตัวอ้างอิงไปยัง `x`
```

### &mut T (Mutable Reference)
ตัวอ้างอิงไปยังค่าและสามารถเปลี่ยนแปลงค่าที่ถูกอ้างอิงได้
```rust
let mut x = 42;
let y: &mut i32 = &mut x; // ตัวอ้างอิงไปยัง `x` และสามารถเปลี่ยนแปลงค่าได้
*y += 1; // เปลี่ยนค่า x เป็น 43
```

## Custom Types (ชนิดข้อมูลที่ผู้ใช้กำหนดเอง)

### Struct (โครงสร้าง)
ใช้สำหรับสร้างชนิดข้อมูลที่มีฟิลด์หลายแบบ
```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 10, y: 20 };
```

### Enum (เอ็นัม)
ใช้สำหรับสร้างชนิดข้อมูลที่มีตัวเลือกหลายแบบ
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
let dir = Direction::Up;
```

### Option<T>
ชนิดข้อมูลที่สามารถมีค่าหรือไม่มีค่าก็ได้
```rust
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;
```

### Result<T, E>
ชนิดข้อมูลที่ใช้สำหรับจัดการกับการคำนวณที่อาจจะสำเร็จหรือเกิดข้อผิดพลาด
```rust
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y != 0 {
        Ok(x / y)
    } else {
        Err(String::from("Division by zero"))
    }
}
```
















## Basic Rust
    1. ในภาษา Rust คำว่า mut ย่อมาจาก "mutable" ซึ่งหมายความว่า "สามารถเปลี่ยนแปลงได้" หรือ "ปรับเปลี่ยนได้"

# Rust Types Overview

ในภาษา Rust มีหลายประเภทข้อมูล (types) ที่ใช้ในการจัดการกับข้อมูลในโปรแกรม นี่คือประเภทข้อมูลหลัก ๆ ใน Rust พร้อมตัวอย่าง:
