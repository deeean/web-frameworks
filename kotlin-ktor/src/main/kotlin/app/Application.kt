package app

import io.ktor.server.engine.*
import io.ktor.server.netty.*
import app.plugins.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.plugins.contentnegotiation.*

fun main() {
    embeddedServer(Netty, port = 3000, host = "localhost") {
        install(ContentNegotiation) {
            json()
        }
        configureRouting()
    }.start(wait = true)
}
