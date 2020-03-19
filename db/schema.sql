CREATE TABLE providers (
	id INT UNSIGNED NOT NULL PRIMARY KEY,
	email VARCHAR(255) NOT NULL,
	pass_hash CHAR(60) NOT NULL,
	UNIQUE INDEX(user_id),
	UNIQUE INDEX(email)
);

CREATE TABLE users (
	id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	name VARCHAR(255) NOT NULL
);

CREATE TABLE users_paymethods (
	user_id INT UNSIGNED NOT NULL PRIMARY KEY,
	method VARCHAR(255) NOT NULL PRIMARY KEY,
	method_info TEXT NOT NULL,
	INDEX(method)
);

CREATE TABLE meals (
	id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	user_id INT UNSIGNED NOT NULL,
	restaurant VARCHAR(255) NOT NULL,
	opened DATETIME NOT NULL,
	closed DATETIME,
	ordered DATETIME,
	acquired DATETIME,
	delivered DATETIME,
	INDEX(user_id)
);

CREATE TABLE orders (
	id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	meal_id INT UNSIGNED NOT NULL,
	user_id INT UNSIGNED NOT NULL,
	paid TINYINT UNSIGNED NOT NULL,
	paid_method VARCHAR(255),
	INDEX(meal_id),
	INDEX(user_id),
	UNIQUE INDEX meal_id__user_id (meal_id, user_id)
);

CREATE TABLE orders_items (
	id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	order_id BIGINT UNSIGNED NOT NULL,
	item VARCHAR(255) NOT NULL,
	price DECIMAL(5, 2) NOT NULL,
	INDEX(order_id)
);

CREATE TABLE sessions (
	cookie VARCHAR(63) NOT NULL PRIMARY KEY,
	provider_id INT UNSIGNED NOT NULL,
	expiry DATETIME NOT NULL,
	INDEX(provider_id)
);

