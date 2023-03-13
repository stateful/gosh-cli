#!/usr/bin/env bats
load 'helpers/bats-support/load'
load 'helpers/bats-assert/load'
load 'helpers/bats-detik/lib/detik'

FILE_PATH=./
RUNME_FLAGS="--chdir $FILE_PATH --filename README.md"
RUNME_RUN_CMD="runme run $RUNME_FLAGS"

@test "Installs latest version of CLI" {
    run $RUNME_RUN_CMD install
    assert_line -p "tonos_cli 0.32.0"
}

@test "Setup network" {
    run $RUNME_RUN_CMD tonoscli-setup-network
    assert_line -p "\"abi_path\": \"SafeMultisigWallet.abi.json\","
    assert_line -p "\"keys_path\": \"key.json\","
    assert_line -p "\"https://mainnet.evercloud.dev\""
}

@test "Clear network" {
    run $RUNME_RUN_CMD clear-network
    assert_line -p "\"url\": \"net.evercloud.dev\","
}

@test "Reset network" {
    run $RUNME_RUN_CMD tonoscli-endpoint-reset
    assert_line -p "\"http://127.0.0.1/\": ["
    assert_line -p "\"main.evercloud.dev\": ["
    assert_line -p "\"net.evercloud.dev\": ["
}

@test "Print network config" {
    run $RUNME_RUN_CMD tonoscli-network-print
    assert_line -p "\"http://127.0.0.1/\": ["
    assert_line -p "\"main.evercloud.dev\": ["
    assert_line -p "\"net.evercloud.dev\": ["
}

@test "Remove network endpoint" {
    run $RUNME_RUN_CMD tonoscli-config-remove-endpoint
    assert_output -p "{
  \"http://127.0.0.1/\": [
    \"http://0.0.0.0\",
    \"http://127.0.0.1\",
    \"http://localhost\"
  ],
  \"net.evercloud.dev\": [
    \"https://devnet.evercloud.dev\"
  ]
}"
}

@test "Add network endpoint" {
    run $RUNME_RUN_CMD tonoscli-config-add-endpoint
    assert_line -p "\"http://127.0.0.1/\": ["
    assert_line -p "\"main.evercloud.dev\": ["
    assert_line -p "\"net.evercloud.dev\": ["
}

@test "Add alias" {
    run $RUNME_RUN_CMD tonoscli-config-add-alias
    assert_line -p "\"abi_path\": \"samples/SafeMultisigWallet.abi.json\""
    assert_line -p "\"key_path\": \"key0.keys.json\""
}

@test "Print alias" {
    run $RUNME_RUN_CMD tonoscli-config-alias-print
    assert_line -p "\"abi_path\": \"samples/SafeMultisigWallet.abi.json\""
    assert_line -p "\"key_path\": \"key0.keys.json\""
}

@test "Generate public key" {
    run $RUNME_RUN_CMD tonoscli-genpubkey
    assert_line -p "\"Public key\": \""
}

@test "Run contract" {
    run $RUNME_RUN_CMD tonoscli-run-contract
    assert_line -p "\"stake\": \"65535\","
}

@test "Decode account data" {
    run $RUNME_RUN_CMD tonoscli-decode-account
    assert_line -p "\"a\": \"I like it.\","
}