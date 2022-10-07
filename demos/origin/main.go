package main

import (
	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	app.Post("/join", func(c *fiber.Ctx) error {
		return c.SendString("you just posted to /join")
	})
	app.Get("/nested/route", func(c *fiber.Ctx) error {
		return c.SendString("you just getted /nested/route")
	})

	app.Listen(":9999")
}
