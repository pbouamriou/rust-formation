#pragma once

#include <string>
#include <memory>

namespace CppTest
{
    class MyConfig
    {
    public:
        void setName(const std::string& name);
        std::string name() const;

        void setWidth(uint32_t width);
        uint32_t width() const;

        void setHeight(uint32_t height);
        uint32_t height() const;

        static std::unique_ptr<MyConfig> from_file(const std::string& filename);

    private:
        std::string _name = "";
        uint32_t _width = 0;
        uint32_t _height = 0;
    };

}
