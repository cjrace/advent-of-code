#include <iostream>
#include <fstream>
#include <string>
#include <vector>

int main()
{
    std::ifstream file("day4_input.txt");
    // std::ifstream file("day4_test_input.txt");
    if (!file.is_open())
    {
        std::cerr << "Error opening file" << std::endl;
        return 1;
    }

    // Count number instances of xmas on each line left to right
    std::string line;
    int xmas_count = 0;
    while (std::getline(file, line))
    {
        size_t pos = line.find("XMAS");
        while (pos != std::string::npos)
        {
            xmas_count++;
            pos = line.find("XMAS", pos + 4);
        }
    }
    std::cout << "Number of xmas after step 1: " << xmas_count << std::endl;

    // Reset file stream to beginning
    file.clear();
    file.seekg(0, std::ios::beg);

    // Count number of instances of SAMX on each line left to right
    while (std::getline(file, line))
    {
        size_t pos = line.find("SAMX");
        while (pos != std::string::npos)
        {
            xmas_count++;
            pos = line.find("SAMX", pos + 4);
        }
    }
    std::cout << "Number of xmas after step 2: " << xmas_count << std::endl;

    // Reset file stream to beginning
    file.clear();
    file.seekg(0, std::ios::beg);

    // Count number of vertical instances of XMAS
    std::vector<std::string> lines;
    while (std::getline(file, line))
    {
        lines.push_back(line);
    }

    int num_lines = lines.size();
    int line_length = lines[0].size();

    for (int col = 0; col < line_length; ++col)
    {
        std::string column;
        for (int row = 0; row < num_lines; ++row)
        {
            column += lines[row][col];
        }

        size_t pos = column.find("XMAS");
        while (pos != std::string::npos)
        {
            xmas_count++;
            pos = column.find("XMAS", pos + 4);
        }
    }
    std::cout << "Number of xmas after step 3: " << xmas_count << std::endl;

    // Reset file stream to beginning
    file.clear();
    file.seekg(0, std::ios::beg);
    // Clear lines vector before reusing
    lines.clear();
    // Count number of vertical instances of SAMX
    while (std::getline(file, line))
    {
        lines.push_back(line);
    }

    num_lines = lines.size();
    line_length = lines[0].size();

    for (int col = 0; col < line_length; ++col)
    {
        std::string column;
        for (int row = 0; row < num_lines; ++row)
        {
            column += lines[row][col];
        }

        size_t pos = column.find("SAMX");
        while (pos != std::string::npos)
        {
            xmas_count++;
            pos = column.find("SAMX", pos + 4);
        }
    }

    std::cout << "Number of xmas after step 4: " << xmas_count << std::endl;

    // Reset file stream to beginning
    file.clear();
    lines.clear();
    file.seekg(0, std::ios::beg);
    // Count diagonal instances of XMAS from top left to bottom right
    for (int row = 0; row <= num_lines - 4; ++row)
    {
        for (int col = 0; col <= line_length - 4; ++col)
        {
            bool found = true;
            for (int k = 0; k < 4; ++k)
            {
                if (lines[row + k][col + k] != "XMAS"[k])
                {
                    found = false;
                    break;
                }
            }
            if (found)
            {
                xmas_count++;
            }
        }
    }

    std::cout << "Number of xmas after step 5: " << xmas_count << std::endl;

    // Reset file stream to beginning
    file.clear();
    lines.clear();
    file.seekg(0, std::ios::beg);
    // Count diagonal instances of SAMX from top left to bottom right
    for (int row = 0; row <= num_lines - 4; ++row)
    {
        for (int col = 0; col <= line_length - 4; ++col)
        {
            bool found = true;
            for (int k = 0; k < 4; ++k)
            {
                if (lines[row + k][col + k] != "SAMX"[k])
                {
                    found = false;
                    break;
                }
            }
            if (found)
            {
                xmas_count++;
            }
        }
    }

    std::cout << "Number of xmas after step 6: " << xmas_count << std::endl;

    // Reset file stream to beginning
    file.clear();
    lines.clear();
    file.seekg(0, std::ios::beg);
    // Count diagonal instances of XMAS from top right to bottom left
    for (int row = 0; row <= num_lines - 4; ++row)
    {
        for (int col = 3; col < line_length; ++col)
        {
            bool found = true;
            for (int k = 0; k < 4; ++k)
            {
                if (lines[row + k][col - k] != "XMAS"[k])
                {
                    found = false;
                    break;
                }
            }
            if (found)
            {
                xmas_count++;
            }
        }
    }

    std::cout << "Number of xmas after step 7: " << xmas_count << std::endl;

    // Reset file stream to beginning
    file.clear();
    lines.clear();
    file.seekg(0, std::ios::beg);
    // Count diagonal instances of SAMX from top right to bottom left
    for (int row = 0; row <= num_lines - 4; ++row)
    {
        for (int col = 3; col < line_length; ++col)
        {
            bool found = true;
            for (int k = 0; k < 4; ++k)
            {
                if (lines[row + k][col - k] != "SAMX"[k])
                {
                    found = false;
                    break;
                }
            }
            if (found)
            {
                xmas_count++;
            }
        }
    }

    std::cout << "Final count of xmas: " << xmas_count << std::endl;
    file.close();
    return 0;
}