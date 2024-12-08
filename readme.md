
## Usage

使用rust实现的clang-format配置文件生成工具，可以生成clang-format的配置文件，也可以生成格式化文件的模版。

### 生成格式化文件模版

```bash
cargo run -- --reference template.cpp
```

会生成一个`template.cpp`文件，内容如下：
```c++

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

```

### 生成clang-format文件

```bash
cargo run -- template.cpp .clangformat 160
```

生成的 .clangformat 文件内容如下：
```bash
# created with clang-format-cfg-generator-rs
# created for clang-format version 16.0

Language: Cpp

UseTab: Never
IndentWidth: 4
ColumnLimit: 107
MaxEmptyLinesToKeep: 1
FixNamespaceComments: true

BreakBeforeBraces: Custom
BraceWrapping:
  AfterClass: true
  AfterFunction: true
  AfterNamespace: true
  AfterStruct: true
  AfterControlStatement: true
  AfterEnum: true
  BeforeElse: true

SpacesInConditionalStatement: false

SpaceBeforeAssignmentOperators: true
SpacesInParentheses: false
SpaceBeforeSquareBrackets: false
PointerAlignment: Left
ReferenceAlignment: Pointer

SpaceBeforeParens: Custom
SpaceBeforeParensOptions:
  SpaceBeforeSquareBrackets: false
  SpaceBeforeAssignmentOperators: true

```

## 其他工具

### clang-format-diff.py

```bash
// 格式化最新的commit，并直接在原文件上修改
git diff -U0 HEAD^ | clang-format-diff.py -i -p1
```

## Other

which is inspired by [clang-format-generator](https://github.com/SebastianBach/clang-format-generator).