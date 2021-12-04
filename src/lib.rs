use std::ops::{Add,AddAssign};
#[derive(Debug,Clone)]
pub struct StringPlus{
    my_string:String
}
impl StringPlus{
    pub fn new(src:&str)->Self{
        StringPlus{
            my_string:src.to_string()
        }
    }
    pub fn set_my_string(&mut self,tar:&str)->&mut StringPlus{
        self.my_string=tar.to_string();
        self        
    }
    pub fn get_my_string(&self)->String{
        self.my_string.to_string()
    }
    pub fn string_reverse(src:&str)->String{
        let len=src.len();
        let mut out=String::new();
        for i in 0..len{
            out+=&src[len-i-1..len-i];
        }
        out
    }
    pub fn reverse(&mut self)->&mut StringPlus{
        self.my_string=Self::string_reverse(&self.my_string);
        self
    }
    pub fn str_is_palindrome(src:&str)->bool{
        let len=src.len();
        let left=src[0..len/2].to_owned();
        let right=Self::string_reverse(&src[(len+1)/2..len]);
        if left ==right{
            true
        }else{
            false
        }
    }
    pub fn is_palindrome(&self)->bool{
        Self::str_is_palindrome(&self.my_string)
    }
    pub fn string_suffix(src:&str)->String{
        let mut index=0;
        for i in src.chars(){
            index+=1;
            if i == '.'{
                break
            }
        }
        src[index..src.len()].to_string()
    }
    pub fn suffix(&self)->String{
        Self::string_suffix(&self.my_string)
    }
    pub fn str_to_vec_chars(src:&str)->Vec<char>{
        let mut vec:Vec<char>=vec![];
        let mut index:usize=0;
        for i in src.chars(){
            vec.insert(index, i);
            index+=1;
        }
        vec
    }
    pub fn to_vec_chars(&self)->Vec<char>{
        Self::str_to_vec_chars(&self.my_string)
    }
    pub fn vec_chars_to_u8(src:&Vec<char>)->Vec<u8>{
        let mut v_u8:Vec<u8>=vec![];
        let mut index:usize=0;
        for elem in src.into_iter() {
            v_u8.insert(index, *elem as u8);
            index+=1;
        }
        v_u8
    }
    pub fn vec_u8_to_chars(src:&Vec<u8>)->Vec<char>{
        let mut v_char:Vec<char>=vec![];
        let mut index:usize=0;
        for elem in src.into_iter() {
            v_char.insert(index, *elem as char);
            index+=1;
        }
        v_char

    }
    pub fn vec_chars_to_string(src:&Vec<char>)->String{
        let mut out=String::new();
        for elem in src.into_iter() {
            out+=&String::from(*elem);
        }
        out
    }
    pub fn from_vec_chars(src:&Vec<char>)->Self{
        let tar=Self::vec_chars_to_string(src);
        StringPlus{
            my_string:tar
        }
    }
    pub fn string_sort(src:&str)->String{
        let mut v_u8=Self::vec_chars_to_u8(&Self::str_to_vec_chars(src));
        v_u8.sort();
        Self::vec_chars_to_string(&Self::vec_u8_to_chars(&v_u8))    
    }
    pub fn sort(&mut self)->&mut StringPlus{
        self.my_string=Self::string_sort(&self.my_string);
        self
    }
    pub fn string_is_sort(tar:&str)->bool{
        let temp= Self::string_sort(tar);
        tar == &temp
    }
    pub fn is_sort(&self)->bool{
        let temp=Self::string_sort(&self.my_string);
        self.my_string == temp
    }
    pub fn string_repeat(src:&str,times:usize)->String{
        let mut res=String::new();
        for _ in 0..times{
            res+=src;
        }
        res
    }
    pub fn input_to_buffer(prompt:&str,buffer:&mut String){
        use std::io::Write;
        print!("{}",prompt);
        let _= std::io::stdout().flush();
        std::io::stdin().read_line(buffer).unwrap();
    }
}
impl Add for StringPlus{
    type Output=Self;
    fn add(self,tar:Self)->Self::Output{
        Self{
            my_string:self.my_string+&tar.my_string
        }
        
    }
}
impl Add<&str> for StringPlus{
    type Output=Self;
    fn add(self,tar:&str)->Self::Output{
        Self{
            my_string:self.my_string+tar
        }
    }
}
impl AddAssign for StringPlus{
    fn add_assign(&mut self,tar:StringPlus){
        self.my_string+=&tar.my_string;
        
    }
}
impl AddAssign<&str> for StringPlus{
    fn add_assign(&mut self,tar:&str){
        self.my_string+=tar
    }
}
