package app.plugins

import io.ktor.server.routing.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.Serializable

@Serializable
data class Response(
    val message: String
)

fun Application.configureRouting() {
    routing {
        get("/ping") {
            call.respond(Response("pong"))
        }
    }
}
