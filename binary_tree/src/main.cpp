#include <iostream>
#include <random>
#include <chrono>

struct Node {
    int data;
    Node* left;
    Node* right;

    Node(int data) : data(data), left(nullptr), right(nullptr) {}
};

class BinarySearchTree {
public:
    Node* root;

    BinarySearchTree() : root(nullptr) {}

    void insert(int x) {
        root = insertRecursive(root, x);
    }

    bool search(int x) {
        return searchRecursive(root, x);
    }

    void remove(int x) {
        root = removeRecursive(root, x);
    }

private:
    Node* insertRecursive(Node* node, int x) {
        if (node == nullptr) {
            return new Node(x);
        }

        if (x < node->data) {
            node->left = insertRecursive(node->left, x);
        } else {
            node->right = insertRecursive(node->right, x);
        }

        return node;
    }

    bool searchRecursive(Node* node, int x) {
        if (node == nullptr) {
            return false;
        }

        if (x == node->data) {
            return true;
        } else if (x < node->data) {
            return searchRecursive(node->left, x);
        } else {
            return searchRecursive(node->right, x);
        }
    }

    Node* removeRecursive(Node* node, int x) {
        if (node == nullptr) {
            return nullptr;
        }

        if (x < node->data) {
            node->left = removeRecursive(node->left, x);
        } else if (x > node->data) {
            node->right = removeRecursive(node->right, x);
        } else {
            // Node found
            if (node->left == nullptr) {
                Node* temp = node->right;
                delete node;
                return temp;
            } else if (node->right == nullptr) {
                Node* temp = node->left;
                delete node;
                return temp;
            }

            // Node with two children: Get the inorder successor (smallest in the right subtree)
            Node* successor = findMin(node->right);
            node->data = successor->data;
            node->right = removeRecursive(node->right, successor->data);
        }

        return node;
    }

    Node* findMin(Node* node) {
        while (node->left != nullptr) {
            node = node->left;
        }
        return node;
    }
};

int main() {
    const int N = 10000; // Максимальный размер дерева

    // Создание генератора случайных чисел
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> distrib(0, N * 2); // Диапазон случайных чисел

    // Создание деревьев
    BinarySearchTree tree_random;
    BinarySearchTree tree_sorted;

    // Заполнение дерева в случайном порядке
    auto start_random_insert = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < N; ++i) {
        tree_random.insert(distrib(gen));
    }
    auto end_random_insert = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration_random_insert = end_random_insert - start_random_insert;

    // Заполнение дерева в возрастающем порядке
    auto start_sorted_insert = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < N; ++i) {
        tree_sorted.insert(i);
    }
    auto end_sorted_insert = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration_sorted_insert = end_sorted_insert - start_sorted_insert;

    // Поиск N/10 случайных чисел в каждом дереве
    const int num_searches = N / 10;
    auto start_random_search = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < num_searches; ++i) {
        tree_random.search(distrib(gen));
    }
    auto end_random_search = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration_random_search = end_random_search - start_random_search;

    auto start_sorted_search = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < num_searches; ++i) {
        tree_sorted.search(distrib(gen));
    }
    auto end_sorted_search = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration_sorted_search = end_sorted_search - start_sorted_search;

    // Удаление N/10 случайных элементов в каждом дереве
    const int num_removes = N / 10;
    auto start_random_remove = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < num_removes; ++i) {
        tree_random.remove(distrib(gen));
    }
    auto end_random_remove = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration_random_remove = end_random_remove - start_random_remove;

    auto start_sorted_remove = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < num_removes; ++i) {
        tree_sorted.remove(distrib(gen));
    }
    auto end_sorted_remove = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration_sorted_remove = end_sorted_remove - start_sorted_remove;

    // Вывод результатов
    std::cout << "Дерево, заполненное в случайном порядке:" << std::endl;
    std::cout << "  Время вставки: " << duration_random_insert.count() << " s" << std::endl;
    std::cout << "  Время поиска: " << duration_random_search.count() << " s" << std::endl;
    std::cout << "  Время удаления: " << duration_random_remove.count() << " s" << std::endl;

    std::cout << "Дерево, заполненное в возрастающем порядке:" << std::endl;
    std::cout << "  Время вставки: " << duration_sorted_insert.count() << " s" << std::endl;
    std::cout << "  Время поиска: " << duration_sorted_search.count() << " s" << std::endl;
    std::cout << "  Время удаления: " << duration_sorted_remove.count() << " s" << std::endl;

    std::cout << std::endl << "Вывод:" << std::endl;
    std::cout << "  Вставка в случайном порядке быстрее, чем во возрастающем (т.к. дерево вырождается в список)." << std::endl;
    std::cout << "  Поиск и удаление в случайном дереве быстрее, чем в отсортированном (по той же причине)." << std::endl;

    return 0;
}
