package main

import (
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/logger"
)

func main() {
	app := fiber.New()
	app.Use(logger.New(logger.Config{
		Format: "[${ip}]:${port} ${status} - ${method} ${path}\n",
	}))

	app.Post("/join", func(c *fiber.Ctx) error {
		return c.SendString("you just posted to /join")
	})
	app.Get("/nested/route", func(c *fiber.Ctx) error {
		return c.SendString("you just getted /nested/route")
	})

	app.Listen(":9999")
}
