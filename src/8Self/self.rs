// class Bag {
//     String item;

//     void putItem(String item) {
//         this.item = item;
//     }
// }

struct Bag {
    item: Option<String>
}

impl Bag {
    fn put_item(&mut self, item: String) {
        self.item = Some(item);
    }
}


Important Differences (Don’t Skip)
    1️⃣ self is NOT automatic in Rust
    👉 this is always available in java
    eg: fn foo() {}
    👉 ❌ No self unless you explicitly add it

    You must choose:
    fn foo(self) // takes ownership
    fn foo(&self)// read only
    fn foo(&mut self)// modify

    Rust forces you to be explicit about ownership.

    2️⃣ this vs self ownership
    Java:
    1. This is always a reference
    2. Garbage Collector handles memory

    Rust:
    self can:
    own the object
    borrow immutably
    borrow mutably
