/*
When we follow the common rules of ownership --> to calculate the length of the string & for it to not get out of scope after calculating we need to -->
- transfer ownership to length function
- calculate length
- return the string & length to take back the ownership
*/
pub fn ownership_fiasco(s: String) -> (String, usize) {
    let l = s.len(); // ownership taken
    (s, l) // ownership given back
}