// What Clang refuses under `-Wconsumed -Wdangling -Werror`: each case, built alone as
// `-DCASE=n`, breaks a contract `tests/api.cpp` otherwise reaches only at run time, or not
// at all. With no case it builds clean.
#include <optional>
#include <string_view>
#include <utility>

#include "api.hpp"

#ifndef CASE
#define CASE 0
#endif

void broken() {
#if CASE == 1
	// A call on an object moved out of by a method.
	api::Counter c = api::Counter::new_("c", 0);
	api::Label label = std::move(c).into_label(std::nullopt);
	c.get();
#elif CASE == 2
	// A call on an object moved from.
	api::Counter c = api::Counter::new_("c", 0);
	api::Counter d = std::move(c);
	c.get();
#elif CASE == 3
	// A shared object moved from, then moved out of.
	api::Gauge g = api::Gauge::new_(0);
	api::Gauge h = std::move(g);
	std::move(g).into_level();
#elif CASE == 4
	// A shared object moved out of twice.
	api::Gauge g = api::Gauge::new_(0);
	std::move(g).into_level();
	std::move(g).into_level();
#elif CASE == 5
	// A lent object's value moved out: the borrow yields only an lvalue.
	api::Registry registry = api::Registry::new_(api::Counter::new_("c", 0));
	registry.edit([](api::borrow<api::Counter> counter) {
		api::Label label = std::move(counter)->into_label(std::nullopt);
		return false;
	});
#elif CASE == 6
	// A lent object kept past its call: the borrow neither moves nor converts.
	static std::optional<api::Counter> kept;
	api::Registry registry = api::Registry::new_(api::Counter::new_("c", 0));
	registry.edit([](api::borrow<api::Counter> counter) {
		kept.emplace(std::move(counter));
		return false;
	});
#elif CASE == 7
	// A view of a temporary buffer, and its text: both dangle at the semicolon.
	api::view name = api::Counter::new_("c", 0).name();
	std::string_view text = api::Counter::new_("c", 0).name().str();
	(void)name;
	(void)text;
#endif
}
