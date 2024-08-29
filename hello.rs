fn main() {
    // ในทั่วไป `{}` จะถูกแทนที่ด้วยอาร์กิวเมนต์ต่าง ๆ โดยอัตโนมัติ
    // อาร์กิวเมนต์เหล่านี้จะถูกแปลงเป็นสตริง
    println!("{} days", 31);

    // เราสามารถใช้การระบุตำแหน่งของอาร์กิวเมนต์ได้ โดยใส่ตัวเลขภายใน `{}` 
    // เพื่อตั้งค่าอาร์กิวเมนต์ที่จะถูกแทนที่ เริ่มนับจาก 0
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // สามารถใช้ชื่อของอาร์กิวเมนต์ได้เช่นกัน
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // สามารถเรียกการฟอร์แมตที่แตกต่างกันได้โดยใส่อักขระฟอร์แมตตามหลัง `:`
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // เราสามารถจัดตัวเลขให้ชิดขวาด้วยความกว้างที่กำหนดได้ เช่น อันนี้จะให้ผลลัพธ์เป็น "    1"
    println!("{number:>5}", number=1);

    // สามารถเพิ่มเลขศูนย์ข้างหน้าตัวเลขได้
    println!("{number:1>5}", number=1); // 00001
    // และจัดให้ชิดซ้ายได้โดยการสลับเครื่องหมาย อันนี้จะให้ผลลัพธ์เป็น "10000"
    println!("{number:0<5}", number=1); // 10000

    // สามารถใช้ชื่ออาร์กิวเมนต์ในตัวฟอร์แมตได้โดยใส่ `$`
    println!("{number:0>width$}", number=1, width=5);

    // Rust จะตรวจสอบเพื่อให้แน่ใจว่าเราใช้อาร์กิวเมนต์ในจำนวนที่ถูกต้อง
    println!("My name is {0}, {1} {0}", "James", "Bond");
    // FIXME ^ เพิ่มอาร์กิวเมนต์ที่หายไป: "James"

    // มีเฉพาะชนิดข้อมูลที่ติดตั้ง fmt::Display เท่านั้นที่สามารถฟอร์แมตด้วย `{}` ได้
    // ชนิดข้อมูลที่กำหนดเองจะไม่สามารถฟอร์แมตด้วย `{}` ได้โดยอัตโนมัติ

    #[allow(dead_code)] // ปิดการเตือน `dead_code` ที่เตือนเมื่อมีโค้ดที่ไม่ได้ใช้งาน
    struct Structure(i32);

    // บรรทัดนี้จะไม่คอมไพล์เพราะ `Structure` ไม่ได้ติดตั้ง fmt::Display
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ ลองยกเลิกการคอมเมนต์บรรทัดนี้

    // สำหรับ Rust 1.58 และเวอร์ชันที่ใหม่กว่า สามารถจับอาร์กิวเมนต์จากตัวแปรรอบ ๆ ได้โดยตรง
    // เช่นเดียวกับด้านบน อันนี้จะให้ผลลัพธ์เป็น "    1"
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}