package main

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
)

func init() {
	gin.SetMode(gin.ReleaseMode)
}

func main() {
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})

	log.Fatalln(r.Run(":3000"))
}
