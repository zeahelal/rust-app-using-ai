module.exports = {
  apps: [
    {
      name: 'backend',
      script: './target/release/backend',
      exec_mode: 'fork',
      instances: 1,
      env: {
        RUST_LOG: 'info',
      },
    },
  ],
};
