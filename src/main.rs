fn main() {

   struct Block {
     index: u64,
     time_stamp: String,
     data: u128,
     hash: String,
     previous_hash: String
   };
   
   let b = Block {
       index: 0,
       time_stamp: String::from("timestamp"),
       data: 0,
       hash: String::from("hash"),
       previous_hash: String::from("previous hash")
   };

   println!("{}{} {} {} {}",b.index, b.time_stamp, b.data, b.hash, b.previous_hash);
}
