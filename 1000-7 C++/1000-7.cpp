#include <iostream>
int main(){
    int i = 1000;
    while (i > 0 ){
        std::cout << i << " - 7 = " << i - 7 << std::endl;
        i -= 7;
    }
    return 0;
}