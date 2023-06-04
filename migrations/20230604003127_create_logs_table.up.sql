-- Add up migration script here
CREATE TABLE IF NOT EXISTS logs (
    id UUID NOT NULL,
    user_agent VARCHAR(256) NOT NULL,
    response_time INT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT PK_logs PRIMARY KEY (id)
);