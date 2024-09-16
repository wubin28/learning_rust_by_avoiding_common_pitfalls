#include <iostream>
#include <memory>

int main()
{
  std::cout << "C++ 悬垂指针示例开始运行..." << std::endl;

  int* rawPtr = nullptr;

  {
    std::shared_ptr<int> smartPtr = std::make_shared<int>(42);
    rawPtr = smartPtr.get();  // 获取裸指针

    std::cout << "智能指针管理的值: " << *smartPtr << std::endl;
    std::cout << "裸指针指向的值: " << *rawPtr << std::endl;
  }  // smartPtr 在此作用域结束后被销毁，内存被释放

  // 此时 rawPtr 成为悬垂指针
  std::cout << "尝试访问悬垂裸指针的值: " << *rawPtr
            << std::endl;  // 未定义行为，可能崩溃

  return 0;
}
// Output:
// C++ 悬垂指针示例开始运行...
// 智能指针管理的值: 42
// 裸指针指向的值: 42
// 尝试访问悬垂裸指针的值: 0
