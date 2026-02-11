#include <stdexcept>
#include <system_error>
#include "except.hxx"


static thread_local exception_record ex_record;

#if __has_include(<cxxabi.h>)
#include <cxxabi.h>
#include <cstdlib>
static std::string exception_name(const std::exception &ex)
{
     const char* name = typeid(ex).name();
     int status = 0;
     char* demangled = abi::__cxa_demangle(name, nullptr, nullptr, &status);
     std::string result = (status == 0 && demangled) ? demangled : name;
     std::free(demangled);
     return result;
}
#else
static std::string exception_name(const std::exception &ex)
{
     return typeid(ex).name();
}
#endif


const exception_record *save_exception()
{
     ex_record.type_name[0] = 0;
     std::snprintf(ex_record.message,
		   sizeof ex_record.message, "%s",
		   "unnown exception");
     return &ex_record;
}

const exception_record *save_exception(const std::exception &ex)
{
     std::string exname = exception_name(ex);

     std::snprintf(ex_record.type_name,
		   sizeof ex_record.type_name, "%s", exname.c_str());
     std::snprintf(ex_record.message, sizeof ex_record.message,
		   "%s: %s", exname.c_str(), ex.what());
     return &ex_record;
}

const exception_record *save_exception(const std::system_error &ex)
{
     std::string exname = exception_name(ex);

     std::snprintf(ex_record.type_name,
		   sizeof ex_record.type_name, "%s", exname.c_str());
     std::snprintf(ex_record.message, sizeof ex_record.message, "%s: %s %d %s",
		   exname.c_str(), ex.code().message().c_str(),
		   ex.code().value(), ex.what());
     return &ex_record;
}

extern "C" {
     const char *exception_name(const exception_record *ex)
     {
	  return ex->type_name;
     }

     const char *exception_message(const exception_record *ex)
     {
	  return ex->message;
     }
}
