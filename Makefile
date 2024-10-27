access_token := ${ACCESS_TOKEN}
book_id := ${BOOK_ID}
checkout_id := ${CHECKOUT_ID}
auth_header = 'authorization: Bearer ${ACCESS_TOKEN}'

.PHONY: login
login:
	 curl -X POST http://localhost:8080/auth/login -H 'content-type: application/json' -d '{"email": "eleazar.fig@example.com", "password": "passwd"}' | jq

.PHONY: insert-book
insert-book:
	curl -X POST http://localhost:8080/api/v1/books -H 'authorization: Bearer ${ACCESS_TOKEN}' -H 'content-type: application/json' -d '{"title": "Rust book", "author": "me", "isbn": "1234567890", "description": ""}' | jq

.PHONY: list-book
list-book:
	curl http://localhost:8080/api/v1/books -H ${auth_header} | jq

.PHONY: checkout-book
checkout-book:
	curl -X POST http://localhost:8080/api/v1/books/${BOOK_ID}/checkouts -H ${auth_header} | jq

.PHONY: list-checkouts
list-checkouts:
	curl http://localhost:8080/api/v1/books/checkouts -H ${auth_header} | jq

.PHONY: return-book
return-book:
	curl -X PUT http://localhost:8080/api/v1/books/${BOOK_ID}/checkouts/${CHECKOUT_ID}/returned -H ${auth_header} | jq

