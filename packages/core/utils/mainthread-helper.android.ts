export function dispatchToMainThread(func: () => void) {
	new android.os.Handler(android.os.Looper.getMainLooper()).post(
		new java.lang.Runnable({
			run: func,
		})
	);
}

export function isMainThread(): boolean {
	return android.os.Looper.myLooper() === android.os.Looper.getMainLooper();
}

export function dispatchToUIThread(func: () => void) {
	return function (func) {
		if (func) {
			func();
		}
	};
}
