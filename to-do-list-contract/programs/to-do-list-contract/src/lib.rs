use anchor_lang::prelude::*;

declare_id!("AQKt5sgzZWmPMX5AyNB7brNeybRhsDERBF9taeCWGt62");

#[program]
pub mod todo_list {
    use super::*;

    // Initialize the To-Do list for a user
    pub fn initialize_todo_list(ctx: Context<InitializeTodoList>) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        todo_list.initializer = ctx.accounts.user.key(); // Store the creator's public key
        todo_list.tasks = Vec::new(); // Initialize an empty task list
        Ok(())
    }

    // Add a task to the To-Do list
    pub fn add_task(ctx: Context<AddTask>, title: String) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        let user = &ctx.accounts.user;

        // Only the initializer can add tasks ...
        if user.key() != todo_list.initializer {
            return Err(ErrorCode::Unauthorized.into());
        }

        let task = Task {
            title,
            is_done: false,
            timestamp: Clock::get()?.unix_timestamp, // Get current timestamp
        };

        todo_list.tasks.push(task); // Add the new task to the list
        Ok(())
    }

    // Toggle task completion (mark as done/undone)
    pub fn toggle_task_completion(
        ctx: Context<ToggleTaskCompletion>,
        task_index: usize,
    ) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        let user = &ctx.accounts.user;

        // Only the initializer can toggle task completion
        if user.key() != todo_list.initializer {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Ensure the task index is valid
        if task_index >= todo_list.tasks.len() {
            return Err(ErrorCode::InvalidTaskIndex.into());
        }

        let task = &mut todo_list.tasks[task_index];
        task.is_done = !task.is_done; // Toggle the completion status

        Ok(())
    }

    // Delete a task from the To-Do list
    pub fn delete_task(ctx: Context<DeleteTask>, task_index: usize) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        let user = &ctx.accounts.user;

        // Only the initializer can delete tasks
        if user.key() != todo_list.initializer {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Ensure the task index is valid
        if task_index >= todo_list.tasks.len() {
            return Err(ErrorCode::InvalidTaskIndex.into());
        }

        // Remove the task at the specified index
        todo_list.tasks.remove(task_index);

        Ok(())
    }
}

// Define the Task struct (used inside the To-Do list)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Task {
    pub title: String,  // The title/description of the task
    pub is_done: bool,  // Whether the task is completed or not
    pub timestamp: i64, // Timestamp of task creation
}

// Define the To-Do List account
#[account]
pub struct TodoList {
    pub initializer: Pubkey, // The public key of the user who created the To-Do List
    pub tasks: Vec<Task>,    // The list of tasks for the user
}

// Context for initializing the To-Do list
#[derive(Accounts)]
pub struct InitializeTodoList<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4)] // Account space calculation
    pub todo_list: Account<'info, TodoList>, // The To-Do List account
    #[account(mut)]
    pub user: Signer<'info>, // The user who initializes the list
    pub system_program: Program<'info, System>, // Required to create accounts
}

// Context for adding a task
#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(mut)]
    pub todo_list: Account<'info, TodoList>, // The To-Do List account
    #[account(signer)]
    pub user: Signer<'info>, // Only the initializer can add tasks
}

// Context for toggling a task's completion status
#[derive(Accounts)]
pub struct ToggleTaskCompletion<'info> {
    #[account(mut)]
    pub todo_list: Account<'info, TodoList>,
    #[account(signer)]
    pub user: Signer<'info>,
}

// Context for deleting a task
#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(mut)]
    pub todo_list: Account<'info, TodoList>,
    #[account(signer)]
    pub user: Signer<'info>,
}

// Define custom error codes
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,

    #[msg("The task index is invalid.")]
    InvalidTaskIndex,
}
