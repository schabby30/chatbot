function handleChatSubmit(formId, inputId, messagesId, apiUrl) {
    const chatForm = document.querySelector(formId);
    const chatMessages = document.querySelector(messagesId);
    const userInput = document.querySelector(inputId);

    chatForm.addEventListener('submit', (e) => {
        e.preventDefault();
        const userQuery = userInput.value.trim();
        if (userQuery) {
            // Send the query to the backend API
            fetch(apiUrl, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ query: userQuery }),
            })
            .then((response) => response.json())
            .then((data) => {
                chatMessages.innerHTML += `
                    <div class="message">
                        <span>user:</span>
                        <span>${userQuery}</span>
                    </div>
                `;
                userInput.value = '';
                chatMessages.innerHTML += `
                    <div class="message">
                        <span>assistant:</span>
                        <span>${data.response}</span>
                    </div>
                `;
            })
            .catch((error) => console.error(error));
        }
    });
}

function handleSystemPromtSubmit(formId, inputId, messagesId, apiUrl) {
    const chatForm = document.querySelector(formId);
    const chatMessages = document.querySelector(messagesId);
    const userInput = document.querySelector(inputId);

    chatForm.addEventListener('submit', (e) => {
        e.preventDefault();
        const userQuery = userInput.value.trim();
        if (userQuery) {
            // Send the query to the backend API
            fetch(apiUrl, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ model: "claude", prompt: userQuery }),
            })
            .then((response) => response.json())
            .then((data) => {
                chatMessages.innerHTML += `
                    <div class="message">
                        <span>user:</span>
                        <span>${userQuery}</span>
                    </div>
                `;
                userInput.value = '';
                if (response.body == null || response.body == "") {
                    chatMessages.innerHTML += `
                    <div class="message">
                        <span>assistant:</span>
                        <span>${data.response}</span>
                    </div>
                    `;
                } else {
                    chatMessages.innerHTML += `
                    <div class="message">
                        <span>assistant:</span>
                        <span>system prompt added</span>
                    </div>
                    `;
                };
            })
            .catch((error) => console.error(error));
        }
    });
}

// Initialize chat functionality for both chatbots
handleChatSubmit('#form1', '#user-input1', '#messages1', 'http://localhost:5000/ask_groq');
handleSystemPromtSubmit('#form2', '#user-input2', '#messages2', 'http://localhost:5000/add_system_prompt');
