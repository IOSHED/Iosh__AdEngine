from enum import Enum
from typing import Any, Dict

from aiogram_dialog import DialogManager


class CalkStepsHandler:
    """Handler for calculating and tracking dialog steps.

    This class manages step calculations for a dialog flow based on an enum of steps.
    It tracks the current step number and calculates progress through the dialog flow.

    Attributes:
        enum_steps (Enum): Enumeration defining the possible dialog steps.
        step (int, optional): Current step number in the dialog flow. Defaults to None.
    """

    def __init__(
        self,
        enum_steps: Enum,
        step: int = None,
    ) -> None:
        """Initialize the steps handler.

        Args:
            enum_steps (Enum): Enumeration containing all possible dialog steps.
            step (int, optional): Initial step number. Defaults to None.
        """
        self.enum_steps = enum_steps
        self.step = step

    async def get_steps(
        self,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        """Calculate current dialog step metrics.

        Determines the number of completed steps and total steps in the dialog flow.
        If no step is set, calculates it based on dialog data length plus 1.

        Args:
            dialog_manager (DialogManager): The dialog manager instance containing dialog state.
            **_kwargs: Additional keyword arguments (unused).

        Returns:
            Dict[str, Any]: Dictionary containing:
                - num_finish_steps (int): Current step number in the dialog flow
                - all_steps (int): Total number of steps defined in enum_steps
        """

        if self.step is None:
            self.step = 1 + len(dialog_manager.dialog_data)

        return {
            "num_finish_steps": self.step,
            "all_steps": len(self.enum_steps),
        }
