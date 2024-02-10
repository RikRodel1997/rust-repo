# 3 mini projects
After having completed section `I Getting Started` in the book 'The Rust Programming Language' I asked Microsoft Copilot for 3 mini projects I could do.

## 1 - Hello, [name]
Description: Create a simple command-line tool that greets the user with a personalized message.
Functional Requirements:
    * The user should be able to choose one of 4 languages.
    * The user should be able to input their name.
    * The program should respond with a friendly greeting like “Hello, [user’s name]!”.
    * Handle cases where the user doesn’t provide a name or provides an empty name.

## 2 - To-Do List Manager:
Description: Develop a basic to-do list manager that allows users to add, view, and mark tasks as completed.
Functional Requirements:
    * Users can add tasks to the list.
    * Display the current list of tasks.
    * Mark tasks as completed.
    * Handle invalid input gracefully (e.g., if the user enters an invalid task number).

## 3 - URL Shortener
Functional Requirements:
* Shortening URLs:
    * The user should be able to input a long URL.
    * Generate a unique short code (e.g., a random alphanumeric string) for the given URL.
    * Save the mapping between the short code and the original URL (e.g., in a text file).
* Expanding Short URLs:
    * When a user provides a short URL, look up the corresponding long URL from the saved mappings.
    * Redirect the user to the original long URL.
* Error Handling:
    * Handle cases where the short URL doesn’t exist or the user provides an invalid input.
    * Display appropriate error messages.
