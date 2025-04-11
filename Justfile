# install the frontend
install:
    @echo "Installing frontend..."
    cd web && pnpm install

# build the frontend application
build-frontend:
    @echo "Building frontend..."
    cd web && pnpm build

# build the backend application
build-backend:
    @echo "Building backend..."
    cd server && cargo build -r

# build all applications
build: build-frontend build-backend

# run the frontend application
run-frontend: install
    @echo "Running frontend..."
    cd web && pnpm dev

# run the backend application
run-backend:
    @echo "Running backend..."
    cd server && cargo run

# test the backend
test-backend:
    @echo "Testing backend..."
    cd server && cargo test

    # test both
test: test-backend
