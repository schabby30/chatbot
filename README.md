## Rust based Customer Service AI chatbot
It's a Rust based Customer Service AI chatbot with Groq - Claude. 

> **Note:** You must put your Groq API key to the **.env** file.

### Install and run
The app runs in Docker containers. To run it, in the project's folder type:
```$ ./run.sh```
To stop it, run:
```$ docker-compose down```

### Frontend
You can interact with the AI assistant,
or you can add system prompts.
You can launch the frontend in you browser at
```http://localhost:5000```

You can ask the assistant about its work at F2 Komplex Kft
and you can ask it about the company's product and prices.

> **Note:** Hungarian only!

### Database
You can watch the live MongoDB on
```
http://localhost/47081
```

### Configuration
The ```.env``` file must contain the following variables:
OPENAI_API_KEY = ***
GROQ_API_KEY = ***
MONGODB_URI = mongodb://mongo:mongo@mongo-db:27017/
DEFAULT_SYSTEM_PROMPT = ***

### Other
You can change the assistant's job by changing the ```DEFAULT_SYSTEM_PROMPT```
in the ```.env``` file and by changing the the ```arlista.csv``` file.

