fn main(){
  let org_arr:[i32;8] = [1, 2, 3, 5, 6, 8, 10, 11];
  let sub_arr:[i32;3] = [6, 8, 10];
  compare_sub_arrays(&org_arr, &sub_arr);
}
    
fn compare_sub_arrays(a: &[i32], b: &[i32]) -> bool {
  let mut count = 0;
  let mut result = false;
  print!("Sub-array exists: ");
  for x in a.iter(){
    for y in b.iter(){
      if x == y {
        print!("{} ",x);
        count += 1
      }
    }
  }
  if count == b.len() {
    result = true;
  }
  result
}
//https://docs.google.com/forms/d/e/1FAIpQLSdPiOhmwpAC_MsSBV1oge2Kkub6WLJLDWuzczhBiKxFQlOzRQ/viewform
