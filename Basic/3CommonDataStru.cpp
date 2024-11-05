# include <iostream>
# include <stack>
# include <queue>
# include <unordered_map>
# include <map>
# include <set>

using namespace std;

int main(){
    // Array
    int arr[5] = {1, 2, 3, 4, 5};
    cout << arr[0] << endl; // output the first element

    // Struct
    struct Person{
        string name;
        int age;
    };
    Person p = {"Tom", 20};
    cout << p.name << endl; // output the name of the person, and output a blank line

    // Class
    // 和java的class定义有点像
    // class 可以理解为除了struct还有函数，即你可以利用这个struct里的数据作为哪些函数的输入，很方便的实现功能，一个class有一些特定的功能，比如dog就是可以bark 
    class Student{
        private:
            string name;
            int age;
        
        public:
            // 构造函数
            Student(string n , int a): name(n), age(a){};
            // 定义成员函数
            void printInfo(){
                cout << "Name: "<<name<<", Age: "<<age << endl;
            }
    };
    Student s("Bob", 18);
    s.printInfo();

    // 链表linked list（单向链表）
    // 本质上也是一个struct
    struct Node{
        int data;
        Node* next;
    };
    Node* head = nullptr; // Node*类型的head是一个空指针
    Node* newNode = new Node{10, nullptr}; // 创建一个新的Node
    head = newNode; // 将head指向这个新的Node

    // 栈stack
    stack<int> st;
    st.push(1);
    st.push(2);
    cout << st.top() << endl; // output the top element
    st.pop(); // pop the top element
    cout << st.top() << endl; // output the top element (already changed)

    // 队列queue
    // 和栈stack类似，只是队列是先进先出，而栈是先进后出
    queue<int> q;
    q.push(11);
    q.push(12);
    cout << q.front() << endl; // output the front element
    q.pop(); // pop the front element
    cout << q.front() << endl; // output the front element (already changed)

    // 双向队列deque
    // deque是double-ended queue的缩写，即双向队列
    // 可以在队列的两端进行插入和删除操作，是栈和队列的结合体
    deque<int> dq;
    dq.push_back(100);
    dq.push_front(101);
    dq.push_front(102);
    cout << dq.front() << endl; // output the front element
    cout << dq.back() << endl; // output the back element
    dq.pop_front(); // pop the front element
    cout << dq.front() << endl; // output the front element (already changed)

    //哈希表hash table
    // 哈希表是一种数据结构，它可以快速的插入和查找数据
    // 不保证元素顺序，有点像字典
    unordered_map<string, int> hashTable;
    hashTable["apple"] = 10;
    cout << hashTable["apple"] << endl;

    // 映射map
    // map是一种有序的哈希表，它可以快速的插入和查找数据
    map<string, int> mymap;
    mymap["apple"] = 10;
    cout << mymap["apple"] << endl;

    // 集合set
    // 保证元素不充分且有序
    set <int> myset;
    myset.insert(11);
    myset.insert(22);
    cout << *myset.begin() << endl; // output the first element

    // 动态数组vector
    // 动态扩展容量，支持随机访问
    vector<int> v;
    v.push_back(1);
    v.push_back(2);
    cout << v[0] << endl; // output the first element 1

    return 0;
}






