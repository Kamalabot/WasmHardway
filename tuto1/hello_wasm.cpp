#include <iostream>

extern "C" {
	void hello_world(){
		std::cout << "Hello Wasm" << std::endl;
	}
}
