fn last_char_of_first_line(text: &str) -> Option<char> {
    // 1. text.lines()返回一个Lines对象，它本质上是一个迭代器
    // 2. 迭代器的next()方法返回一个Option<&str>
    // 3. str的方法chars()返回一个Chars对象，它本质上是一个迭代器
    // 4. 迭代器的last()方法返回一个Option<char>
    text.lines().next()?.chars().last()
}
