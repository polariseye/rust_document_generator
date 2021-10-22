/// example
/// ````
/// /// this is the header of one api document
/// /// + multiple line can start with "+" to preserve extra white space
/// #[rust_document_generator::doc_header]
/// fn main(){
/// }
///
/// /// Module the name of this module
/// /// fn HelloWorld /v1/HelloWorld post # this is figure out the api base info, such as "fn {ApiName} {RequestPath} {HttpMethod} {Description}"
/// /// + api description can have multiple line. and it can start with "+" to preserve extra white space
/// /// param
/// ///     Name    string  required    #this is the param info. such as "{ParamName}   {ParamType} {required/optional} {description}". the description is only one line
/// /// return # this is the return segment. such as "reutrn {return description} \r\n {return content}"
/// ///  +{
/// ///  +  "Desc":"String 其他描述"
/// ///  +}
/// ///  +
/// #[rust_document_generator::api]
/// fn hello_word(name:String,extra_hellor:String)->String{
///     format!("hello world {}",&name)
/// }
/// ````
pub use doc_def::*;
pub use doc_macro::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
