static REF_CONTENT: &str = r#"
///////////////////////////////////// MAX WIDTH ///////////////////////////////////////////////////////////

namespace lib
{

enum TYPES
{
    TYPE_A,
    TYPE_B,
};

struct test_data 
{
    float data;
};

class ReferenceClass 
{
public:
    ReferenceClass(int* value, float& ref)
    {
        if (value) 
        {
            int a = 5;
        }
        else
        {
            int a = 6;
        }
    }
private:
    int values[5];
};

} // namespace lib
"#;

pub(crate) fn generate_reference_file(lines: &mut Vec<String>) {
    
    for line in REF_CONTENT.lines() {
        lines.push(line.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_reference() {
        let mut lines = Vec::new();
        generate_reference_file(&mut lines);
        assert!(lines.len() > 0);
    }
}