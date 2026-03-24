/* ═══════════════════════════════════════════════════════════════
   APP.JS — Client-side Logic
   
   Minimal JavaScript that complements HTMX:
   - Toast notification management (auto-dismiss, stack)
   - HTMX event listeners for error handling
   - Global keyboard shortcuts
   
   This is the ONLY custom JS file. Everything else is handled
   by HTMX attributes in the HTML templates.
═══════════════════════════════════════════════════════════════ */

(function () {
    "use strict";

    // ── Constants ────────────────────────────────────────────
    const TOAST_DURATION = 5000;     // Auto-dismiss after 5 seconds
    const TOAST_FADE_MS  = 250;      // Fade-out animation duration
    const MAX_TOASTS     = 4;        // Max visible toasts at once

    // ── Toast Management ─────────────────────────────────────
    // Toasts are created by HTMX error responses (swapped into
    // #toast-container) or manually via showToast().

    /**
     * Creates and displays a toast notification.
     * @param {string} message - Text to display.
     * @param {"error"|"success"|"warning"} type - Visual style.
     */
    function showToast(message, type) {
        var container = document.getElementById("toast-container");
        if (!container) return;

        // Enforce max visible toasts — remove oldest if at limit.
        while (container.children.length >= MAX_TOASTS) {
            container.removeChild(container.firstElementChild);
        }

        var toast = document.createElement("div");
        toast.className = "toast toast--" + type;
        toast.setAttribute("role", "alert");
        toast.textContent = message;
        container.appendChild(toast);

        // Auto-dismiss after TOAST_DURATION.
        setTimeout(function () {
            dismissToast(toast);
        }, TOAST_DURATION);

        // Click to dismiss immediately.
        toast.addEventListener("click", function () {
            dismissToast(toast);
        });
    }

    /**
     * Fades out and removes a toast element.
     * @param {HTMLElement} toast
     */
    function dismissToast(toast) {
        toast.style.animation =
            "toast-out " + TOAST_FADE_MS + "ms var(--ease-default) forwards";
        setTimeout(function () {
            if (toast.parentNode) {
                toast.parentNode.removeChild(toast);
            }
        }, TOAST_FADE_MS);
    }

    // Expose globally so templates can call it if needed.
    window.showToast = showToast;

    // ── HTMX Event Listeners ─────────────────────────────────

    /**
     * After every HTMX swap, check if the swapped content is a
     * toast (has .toast class). If so, set up auto-dismiss.
     */
    document.addEventListener("htmx:afterSwap", function (event) {
        var target = event.detail.target;
        if (!target) return;

        // If the target IS the toast container, auto-dismiss new children.
        if (target.id === "toast-container") {
            var toasts = target.querySelectorAll(".toast");
            toasts.forEach(function (toast) {
                setTimeout(function () {
                    dismissToast(toast);
                }, TOAST_DURATION);
                toast.addEventListener("click", function () {
                    dismissToast(toast);
                });
            });
        }

        // If an auth-error div received a toast, auto-dismiss it.
        if (target.id === "auth-error") {
            var errorToasts = target.querySelectorAll(".toast");
            errorToasts.forEach(function (toast) {
                setTimeout(function () {
                    dismissToast(toast);
                }, TOAST_DURATION);
            });
        }
    });

    /**
     * Handle HTMX response errors (network failures, 500s).
     * Shows a generic toast so the user knows something went wrong
     * even if the server didn't return an error fragment.
     */
    document.addEventListener("htmx:responseError", function (event) {
        var status = event.detail.xhr ? event.detail.xhr.status : 0;
        var message;

        if (status === 0) {
            message = "NETWORK ERROR — CHECK CONNECTION";
        } else if (status === 401) {
            message = "SESSION EXPIRED — PLEASE LOG IN";
            // Redirect to login after a short delay.
            setTimeout(function () {
                window.location.href = "/auth/login";
            }, 1500);
        } else if (status === 403) {
            message = "ACCESS DENIED — INSUFFICIENT PERMISSIONS";
        } else if (status >= 500) {
            message = "SYSTEM ERROR — PLEASE TRY AGAIN";
        } else {
            message = "REQUEST FAILED [" + status + "]";
        }

        showToast(message, "error");
    });

    /**
     * Handle HTMX send errors (request couldn't be sent at all).
     */
    document.addEventListener("htmx:sendError", function () {
        showToast("CONNECTION FAILED — SERVER UNREACHABLE", "error");
    });

    // ── Page Load Animations ─────────────────────────────────
    // Add staggered fade-in to elements with .animate-in class.
    document.addEventListener("DOMContentLoaded", function () {
        var elements = document.querySelectorAll(".animate-in");
        elements.forEach(function (el, index) {
            el.style.animationDelay = (index * 80) + "ms";
        });
    });

    // ── Keyboard Shortcuts ───────────────────────────────────
    document.addEventListener("keydown", function (event) {
        // ESC — close modals
        if (event.key === "Escape") {
            var modal = document.querySelector(".modal-overlay");
            if (modal) {
                modal.remove();
            }
        }

        // "/" — focus search bar (if exists and not in an input)
        if (
            event.key === "/" &&
            document.activeElement.tagName !== "INPUT" &&
            document.activeElement.tagName !== "TEXTAREA"
        ) {
            var searchInput = document.querySelector(".search-bar__input");
            if (searchInput) {
                event.preventDefault();
                searchInput.focus();
            }
        }
    });
})();