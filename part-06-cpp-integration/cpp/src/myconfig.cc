#include "myconfig.h"

#include <filesystem>
#include <stdexcept>

namespace CppTest
{

    void MyConfig::setName(const std::string &name)
    {
        _name = name;
    }

    std::string MyConfig::name() const
    {
        return _name;
    }

    void MyConfig::setWidth(uint32_t width)
    {
        _width = width;
    }

    uint32_t MyConfig::width() const
    {
        return _width;
    }

    void MyConfig::setHeight(uint32_t height)
    {
        _height = height;
    }

    uint32_t MyConfig::height() const
    {
        return _height;
    }

    std::unique_ptr<MyConfig> MyConfig::from_file(const std::string &filename)
    {
        auto ptr = std::make_unique<MyConfig>();
        if (std::filesystem::exists(filename))
        {
            ptr->setName("Test");
            ptr->setWidth(12);
            ptr->setHeight(24);
        }
        else
        {
            throw std::invalid_argument("File not found");
            // ptr->setName("Unset");
        }

        return ptr;
    }

}
