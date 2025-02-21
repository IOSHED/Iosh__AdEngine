from enum import Enum
from typing import Any, Dict

from aiogram_dialog import DialogManager


class CalkStepsHandler:
    """Handler for calculating and tracking dialog steps.

    This class manages step calculations for a dialog flow based on an enum of steps.

    Attributes:
        enum_steps (Enum): Enumeration defining the possible dialog steps.
    """

    def __init__(
        self,
        enum_steps: Enum,
        step: int = None,
    ) -> None:
        """Initialize the steps handler.

        Args:
            enum_steps (Enum): Enumeration containing all possible dialog steps.
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

        Args:
            dialog_manager (DialogManager): The dialog manager instance containing dialog state.
            **_kwargs: Additional keyword arguments (unused).

        Returns:
            Dict[str, Any]: Dictionary containing:
                - num_finish_steps (int): Number of completed steps plus 1
                - all_steps (int): Total number of possible steps
        """

        if self.step is None:
            self.step = 1 + len(dialog_manager.dialog_data)

        return {
            "num_finish_steps": self.step,
            "all_steps": len(self.enum_steps),
        }
