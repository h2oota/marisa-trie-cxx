#pragma once
#include <stdexcept>
#include <system_error>

struct exception_record {
     char type_name[64];
     char message[512];
};

const exception_record* save_exception();
const exception_record* save_exception(const std::exception &ex);
const exception_record* save_exception(const std::system_error &ex);
