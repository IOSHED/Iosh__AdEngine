from typing import Any, Callable, Dict

from aiogram_dialog import DialogManager


class GettingExecutor:
    """
    A class that executes multiple functions and aggregates their results.

    This executor takes multiple async functions that process dialog manager data
    and combines their results into a single dictionary.
    """

    def __init__(
        self, *functions_injection: Callable[[DialogManager], Dict[str, Any]]
    ) -> None:
        """
        Initialize the executor with a variable number of functions.

        Args:
            *functions_injection: Variable number of async callables that take a DialogManager
                                and return Dict[str, Any]
        """
        self.functions = functions_injection

    async def execute(self, dialog_manager: DialogManager, **kwargs) -> Dict[str, Any]:
        """
        Execute all injected functions and merge their results.

        Args:
            dialog_manager: The dialog manager instance to pass to each function
            **kwargs: Additional keyword arguments to pass to each function

        Returns:
            Dict[str, Any]: Combined dictionary containing results from all functions
        """
        result = {}
        for func in self.functions:
            data = await func(dialog_manager, **kwargs)
            result.update(data)
        return result
