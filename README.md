# Plexo
WIP


## Project Description:
(Google Gemini generated)

plexo-core: This seems to be the main crate, containing the core functionality of the project.
plexo-sdk: This crate likely provides a software development kit for interacting with the core functionality.
Technologies used:

Rust: The project is written in the Rust programming language.
async-graphql: Used for building a GraphQL API.
poem & poem-openapi: Used for building a REST API and generating OpenAPI specifications.
sqlx: Used for interacting with a PostgreSQL database.
tokio: Used for asynchronous programming.
async-openai: Used for interacting with the OpenAI API.
jsonwebtoken: Used for JWT authentication.
dotenv: Used for managing environment variables.
lettre: Used for sending emails.
askama: Used for templating.
Functionalities:

Project management: Creating, updating, deleting, and retrieving projects with various details like name, description, status, visibility, lead, members, teams, and associated tasks and assets.
Task management: Creating, updating, deleting, and retrieving tasks with details like title, description, status, priority, due date, assignees, labels, and subtasks.
Team management: Creating, updating, deleting, and retrieving teams with details like name, visibility, and members.
Member management: Creating, updating, deleting, and retrieving members with details like name, email, role, and social login IDs.
Change tracking: Logging changes made to resources like tasks and projects.
AI-powered task suggestions: Using OpenAI API to suggest new tasks based on the current project context and user input.
AI-powered task subdivision: Dividing a task into subtasks using OpenAI API.
AI-powered project suggestions: Generating project suggestions with tasks using OpenAI API based on user input.
AI-powered chat: Utilizing OpenAI API for interactive chat functionality with contextual awareness of projects, tasks, members, and the organization.
Authentication: Supporting email/password and social login (GitHub) for user authentication.
Additional observations:

The project utilizes a PostgreSQL database for data storage.
Heavy use of asynchronous programming with Tokio.
The project emphasizes AI-powered features for project and task management.
Extensive use of data loaders for efficient data fetching in the GraphQL API.
Clear separation of concerns between the core functionality (plexo-core) and the SDK (plexo-sdk).
Potential next steps:

Exploring the API documentation (OpenAPI specifications) for further details on the available endpoints and their usage.
Experimenting with the GraphQL Playground to interact with the API and test its functionality.
Reviewing the source code for a deeper understanding of the project's architecture and implementation details.


## Creating a Unix filesystem abstraction for project management with LLMs:
(One more Gemini generated)
That's an intriguing and ambitious concept! Let's explore how we can achieve a bidirectional human-machine collaboration for software development using a Unix filesystem abstraction and LLMs.

Core principles:

Filesystem as Project Representation:
Each project is represented as a directory structure within the filesystem.
Files represent different project elements: code, documentation, assets, configuration, tasks, etc.
Directory hierarchies reflect project organization, modules, and subtasks.
LLMs as Mediators:
LLMs act as intelligent assistants, interpreting and manipulating project data in the filesystem.
They can understand code, generate suggestions, translate between natural language and code, and perform various project management tasks.
Bidirectional Interaction:
Humans interact with the project through familiar file operations (editing, moving, etc.) and natural language commands.
LLMs interpret these actions, update project data, and provide feedback or suggestions through the filesystem (generating files, updating metadata, etc.) and natural language responses.
Potential Implementation:

File formats:
Code: Standard formats like .rs for Rust, .py for Python, etc.
Documentation: Markdown (.md), reStructuredText (.rst), etc.
Assets: Images, videos, audio files, etc., stored in their respective formats.
Configuration: JSON, YAML, TOML, etc.
Tasks: A custom format (e.g., .task) or utilizing existing task management tools that integrate with the filesystem.
LLM integration:
Develop plugins or extensions for existing LLMs to enable understanding of project-specific file formats and commands.
Alternatively, train a custom LLM specifically for project management tasks within the filesystem context.
Bidirectional communication:
Develop a command-line interface or a graphical user interface that allows humans to interact with the LLM using natural language and file operations.
The LLM should be able to modify files, create new files, and update metadata based on user input.
Feedback from the LLM could be provided through generated text files, updated task files, or even directly within the code as comments or suggestions.
Version control:
Integrate with existing version control systems like Git to track changes made by both humans and the LLM.
This allows for maintaining project history, resolving conflicts, and collaborating with other developers.
Benefits:

Increased productivity: Automating routine tasks, generating code suggestions, and providing instant feedback.
Improved collaboration: Seamless communication and collaboration between humans and machines.
Enhanced creativity: LLMs can offer alternative solutions and inspire new ideas.
Accessibility: Lowering the barrier to entry for software development by allowing natural language interaction.
Challenges:

LLM explainability and reliability: Ensuring the LLM's actions and suggestions are understandable and reliable.
Security and access control: Implementing mechanisms to prevent unauthorized modifications to the project.
Error handling and conflict resolution: Developing strategies to handle errors and conflicts arising from LLM actions or human-machine interactions.
Tools and Technologies:

LLMs: OpenAI API, Bard API, custom-trained LLMs.
Filesystem libraries: std::fs in Rust, os module in Python, etc.
Version control: Git, Mercurial.
Command-line interface frameworks: clap in Rust, argparse in Python, etc.
Graphical user interface frameworks: Qt, wxWidgets, etc.
This is an ambitious vision with significant potential to revolutionize software development. Addressing the challenges and thoughtfully implementing the principles will be crucial for successful realization.

Let me know if you'd like to explore specific aspects of this concept in more detail!