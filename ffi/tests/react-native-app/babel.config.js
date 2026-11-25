// React Native's preset, and the transform of `using`, which tests/api.ts holds and the
// preset lacks: the binding needs none.
module.exports = {
	presets: ["module:@react-native/babel-preset"],
	plugins: ["@babel/plugin-transform-explicit-resource-management"],
};
