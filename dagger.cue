package dagger

import (
	"dagger.io/dagger"
	"universe.dagger.io/docker"
	"universe.dagger.io/bash"
)

dagger.#Plan & {
	client: {
		filesystem: {
			"./": read: {
				contents: dagger.#FS
				exclude: [
					".cargo",
					".gitx",
					"cue.mod",
					"target",
				]
			}
		}
		env: {
			GITHUB_TOKEN: dagger.#Secret
			CRATESIO_TOKEN: dagger.#Secret
			RELEASE_VERSION: string
		}
	}

	actions: {
		_baseImage: docker.#Build & {
			steps: [
				docker.#Pull & {
					source: "rust:slim"
				},
				docker.#Run & {
					command: {
						name: "apt"
						args: ["update"]
					}
				},
				docker.#Run & {
					command: {
						name: "apt"
						args: ["upgrade", "-y"]
					}
				},
				docker.#Run & {
					command: {
						name: "apt"
						args: ["install", "-y", "git", "wget"]
					}
				},
				docker.#Run & {
					command: {
						name: "rustup"
						args: ["component", "add", "clippy"]
					}
				},
				docker.#Run & {
					command: {
						name: "rustup"
						args: ["target", "add", "x86_64-apple-darwin"]
					}
				},
				docker.#Run & {
					command: {
						name: "wget"
						args: ["https://github.com/cli/cli/releases/download/v2.8.0/gh_2.8.0_linux_amd64.deb", "-P", "/tmp"]
					}
				},
				docker.#Run & {
					command: {
						name: "apt"
						args: ["install", "-y", "/tmp/gh_2.8.0_linux_amd64.deb"]
					}
				},
				docker.#Copy & {
					contents: client.filesystem."./".read.contents
					exclude:  client.filesystem."./".read.exclude
					dest:     "/app"
				}
			]
		}

		hello: bash.#Run & {
			input:   _baseImage.output
			workdir: "/app"
			script: contents: #"""
				uname -a
				cargo version
				"""#
		}

		// Lints the code
		lint: bash.#Run & {
			input:   _baseImage.output
			workdir: "/app"
			script: contents: #"""
				cargo clippy
				"""#
		}

		// Builds the site (debug mode)
		build: bash.#Run & {
			input:   _baseImage.output
			workdir: "/app"
			script: contents: #"""
				cargo build
				"""#
		}

		// Creates a release
		release: bash.#Run & {
			input: _baseImage.output
			workdir: "/app"
			env: {
				GITHUB_TOKEN: client.env.GITHUB_TOKEN
				RELEASE_VERSION: client.env.RELEASE_VERSION
			}
			script: contents: #"""
				gh release create $RELEASE_VERSION --title "Release $RELEASE_VERSION" --generate-notes
				"""#
		}
		
		// Publishes the crate
		publish: bash.#Run & {
			input: _baseImage.output
			workdir: "/app"
			env: {
				CRATESIO_TOKEN: client.env.CRATESIO_TOKEN
			}
			script: contents: #"""
				cargo publish --token $CRATESIO_TOKEN
				"""#
		}
	}
}
