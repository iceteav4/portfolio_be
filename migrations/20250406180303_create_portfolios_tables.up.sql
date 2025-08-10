-- Add up migration script here
-- Create assets table
CREATE TABLE assets (
    id VARCHAR(50) PRIMARY KEY,
    asset_type VARCHAR(20) NOT NULL,
    external_id VARCHAR(50) NOT NULL,
    source VARCHAR(50) NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    image JSONB NOT NULL,
    ext JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create portfolios table
CREATE TABLE portfolios (
    id BIGINT PRIMARY KEY,
    owner_id BIGINT NOT NULL REFERENCES users (id),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create portfolio_assets table
CREATE TABLE portfolio_assets (
    portfolio_id BIGINT NOT NULL REFERENCES portfolios (id),
    asset_id VARCHAR(50) NOT NULL REFERENCES assets (id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    holding_amount DECIMAL NOT NULL DEFAULT 0,
    total_cost DECIMAL NOT NULL DEFAULT 0,
    avg_buy_price DECIMAL NOT NULL DEFAULT 0,
    sold_amount DECIMAL NOT NULL DEFAULT 0,
    total_revenue DECIMAL NOT NULL DEFAULT 0,
    avg_sell_price DECIMAL NOT NULL DEFAULT 0,
    PRIMARY KEY (portfolio_id, asset_id)
);

-- Create transactions table
CREATE TABLE transactions (
    id BIGINT PRIMARY KEY,
    external_id VARCHAR(50),
    portfolio_id BIGINT NOT NULL REFERENCES portfolios (id),
    asset_id VARCHAR(50) NOT NULL REFERENCES assets (id),
    tx_type VARCHAR(20) NOT NULL,
    quantity DECIMAL NOT NULL,
    price DECIMAL NOT NULL,
    fees DECIMAL NOT NULL DEFAULT 0,
    currency VARCHAR(4) NOT NULL,
    notes TEXT,
    executed_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_assets_symbol ON assets (symbol);

CREATE INDEX idx_portfolio_assets_portfolio_id ON portfolio_assets (portfolio_id);

CREATE INDEX idx_transactions_portfolio_id ON transactions (portfolio_id, asset_id, executed_at);
