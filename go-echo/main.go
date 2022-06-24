package main

import (
	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
	"net/http"
)

func main() {
	e := echo.New()
	e.Use(middleware.Recover())

	e.GET("/ping", func(c echo.Context) error {
		return c.JSON(http.StatusOK, map[string]interface{}{
			"message": "pong",
		})
	})

	e.Logger.Fatal(e.Start(":3000"))
}
