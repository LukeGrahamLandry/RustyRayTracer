#ifndef TESTS_EXAMPLE_H
#define TESTS_EXAMPLE_H

class Another;

class Example {
    Example(bool z, float w);
    Example(int x, double y);
    Example();
    ~Example();

    static const Another& create();
};

class Another {
public:
    Another(int x);
    Another(double x) : y{x} {}

    float add(int a, double b) const {
        if (a < b){
            return a + b;
        } else {  // A comment
            return a - b;
        }
    }

    virtual bool* getSomething(const Example& e) = 0;
    int doSomething() const;
    const int& look(int* at, int** my, const int& pointers);

private:
    double y;
};


class YetAnother : public Another {
public:
    bool something;
    YetAnother(const YetAnother& other);
    // bool many[5];
    bool* getSomething(const Example& e) override {
        return &something;
    }
};

#endif
